// Add to Cargo.toml:
// maud = "0.25"

use maud::{Markup, html};
use std::collections::HashSet;

/// Simple props struct with sensible defaults. You can expand this as needed.
#[derive(Clone)]
pub struct Props {
    pub color: String,
    pub seed: String,
    pub goo: f64,
    /// intensity: (x_intensity, y_intensity)
    pub intensity: (f64, f64),
    /// width variance: (x_var, y_var)
    pub width_variance: (f64, f64),
    /// counts: either (x, y) or (top,bottom,right,left)
    pub counts: Option<Counts>,
    pub id_card: String,
    /// border width: either single or per-edge (t,b,r,l)
    pub border_width: BorderWidth,
    pub skew: f64,
    pub edges: Vec<Edge>, // allowed: "top","bottom","left","right"
    pub debug: bool,
    pub content_classes: String,
}

#[derive(Clone)]
pub enum Counts {
    XY(usize, usize),
    TBRL(usize, usize, usize, usize),
}

#[derive(Clone)]
pub enum BorderWidth {
    Single(f64),
    Four { t: f64, b: f64, r: f64, l: f64 },
    WidthHeight(f64, f64),
}

#[derive(Clone, PartialEq)]
pub enum Edge {
    Top,
    Bottom,
    Right,
    Left,
}

impl Edge {
    pub fn to_string(&self) -> String {
        match self {
            Edge::Top => "top".to_string(),
            Edge::Bottom => "bottom".to_string(),
            Edge::Right => "right".to_string(),
            Edge::Left => "left".to_string(),
        }
    }

    pub fn is(&self, side: &str) -> bool {
        self.to_string() == side
    }
}

impl Default for Props {
    fn default() -> Self {
        Props {
            color: "#222222".to_string(),
            seed: "seed".to_string(),
            goo: 3.0,
            intensity: (1.0, 1.0),
            width_variance: (0.0, 0.0),
            counts: None,
            id_card: "goo".to_string(),
            border_width: BorderWidth::Single(1.0),
            skew: 25.0,
            edges: vec![Edge::Top, Edge::Bottom, Edge::Right, Edge::Left],
            debug: false,
            content_classes: "".into(),
        }
    }
}

/// Helper: deterministic "noise" in [-1.0, 1.0] based on seed + coords.
/// This is NOT simplex noise, but it's deterministic and smooth enough for visual variance.
/// We use a simple integer hash (splitmix64-like) then map to [-1,1].
fn seeded_noise(seed: &str, x: i64, y: i64) -> f64 {
    // combine seed into a 64-bit integer hash, then mix with coordinates
    let mut h: u64 = 1469598103934665603u64; // FNV offset basis
    for b in seed.as_bytes() {
        h ^= *b as u64;
        h = h.wrapping_mul(1099511628211u64);
    }
    h = h.wrapping_add((x as u64).wrapping_mul(0x9e3779b97f4a7c15));
    h ^= (y as u64).wrapping_mul(0xbf58476d1ce4e5b9);
    // splitmix64
    let mut z = h.wrapping_add(0x9e3779b97f4a7c15);
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
    z = z ^ (z >> 31);
    // map to [-1,1]
    let f = (z as f64) / (std::u64::MAX as f64);
    f * 2.0 - 1.0
}

/// clamp helper
fn clamp(v: f64, a: f64, b: f64) -> f64 {
    if v.is_nan() {
        a
    } else if v < a {
        a
    } else if v > b {
        b
    } else {
        v
    }
}

/// Build "points" for a given edge, similar to the JS `makePoints`.
/// Each point is (x%, y%) as f64 tuple in range 0..100.
fn make_points(count: usize, direction: &str, props: &Props) -> Vec<(f64, f64)> {
    let mut pts: Vec<(f64, f64)> = Vec::new();
    if count == 0 {
        return pts;
    }
    let step = 100.0 / (count as f64);
    let is_x = direction == "top" || direction == "bottom";
    let is_alt = direction == "bottom" || direction == "right";

    for i in 0..=count {
        let odd = if i % 2 == 0 { 20.0 } else { 0.0 };

        let intensity = if is_x {
            props.intensity.0
        } else {
            props.intensity.1
        };

        // emulate the JS noiseFactory: produce two noise values (noise, noise2)
        // We'll use seeded_noise with different offsets so they differ per side.
        let index = i as i64;
        let seed_base = match direction {
            "top" => format!("{}-top", props.seed),
            "bottom" => format!("{}-bottom", props.seed),
            "right" => format!("{}-right", props.seed),
            "left" => format!("{}-left", props.seed),
            _ => format!("{}-{}", props.seed, direction),
        };

        let noise = seeded_noise(&seed_base, index, index); // analogous to noiseT.value(i,i)
        let noise2 = seeded_noise(&format!("{}-alt", seed_base), index, index);

        let mut adjustment = (noise * 20.0 + odd) * intensity;
        if is_alt {
            adjustment = 100.0 - adjustment;
        }

        // a = clamp(i * step + noise2 * step * (skew/100), 0, 100)
        let a_raw = (i as f64) * step + noise2 * step * (props.skew / 100.0);
        let a = clamp(a_raw, 0.0, 100.0);
        let b = clamp(adjustment, 0.0, 100.0);

        if is_x {
            pts.push((a, b));
        } else {
            pts.push((b, a));
        }
    }

    pts
}

/// Create the structured path data (outer path points per edge and inner offset paths),
/// approximating the logic of createPath.
fn create_path(width: f64, height: f64, props: &Props) -> PathResult {
    // Determine counts
    let (count_x, count_y) = match &props.counts {
        Some(Counts::XY(x, y)) => (*x, *y),
        Some(Counts::TBRL(t, b, r, l)) => {
            // we'll later use the explicit counts
            (0usize, 0usize)
        }
        None => {
            // default fallback similar to JS:
            (
                (width / 20.0).floor() as usize,
                (height / 20.0).floor() as usize,
            )
        }
    };

    // We'll allow props.counts to be TBRL; prefer that if present.
    let (count_top, count_bottom, count_right, count_left) = match &props.counts {
        Some(Counts::TBRL(t, b, r, l)) => (*t, *b, *r, *l),
        Some(Counts::XY(x, y)) => (*x, *y, *x, *y),
        None => (count_x, count_x, count_y, count_y),
    };

    // filter out edges with zero counts
    let mut edges_set: HashSet<String> = props.edges.iter().map(|edge| edge.to_string()).collect();
    if count_top == 0 {
        edges_set.remove("top");
    }
    if count_bottom == 0 {
        edges_set.remove("bottom");
    }
    if count_right == 0 {
        edges_set.remove("right");
    }
    if count_left == 0 {
        edges_set.remove("left");
    }

    let top = if edges_set.contains("top") {
        make_points(count_top, "top", props)
    } else {
        Vec::new()
    };
    let bottom = if edges_set.contains("bottom") {
        make_points(count_bottom, "bottom", props)
    } else {
        Vec::new()
    };
    let right = if edges_set.contains("right") {
        make_points(count_right, "right", props)
    } else {
        Vec::new()
    };
    let left = if edges_set.contains("left") {
        make_points(count_left, "left", props)
    } else {
        Vec::new()
    };

    // ensure closed shape logic (approximate the same operations as the JS code)
    // If top and left both exist, make sure left starts at or above the top start; if not, shift
    let mut left_mod = left.clone();
    let mut right_mod = right.clone();
    let mut top_mod = top.clone();
    let mut bottom_mod = bottom.clone();

    if !top_mod.is_empty() && !left_mod.is_empty() {
        while !left_mod.is_empty() && left_mod[0].1 < top_mod[0].1 {
            left_mod.remove(0);
        }
        left_mod.insert(0, top_mod[0]);
    }

    if !bottom_mod.is_empty() && !left_mod.is_empty() {
        while !left_mod.is_empty() && left_mod[left_mod.len() - 1].1 > bottom_mod[0].1 {
            left_mod.pop();
        }
        left_mod.push(bottom_mod[0]);
    }

    if !bottom_mod.is_empty() && !right_mod.is_empty() {
        while !right_mod.is_empty()
            && right_mod[right_mod.len() - 1].1 > bottom_mod[bottom_mod.len() - 1].1
        {
            right_mod.pop();
        }
        right_mod.push(bottom_mod[bottom_mod.len() - 1]);
    }

    if !top_mod.is_empty() && !right_mod.is_empty() {
        while !right_mod.is_empty() && right_mod[0].1 < top_mod[top_mod.len() - 1].1 {
            right_mod.remove(0);
        }
        right_mod.insert(0, top_mod[top_mod.len() - 1]);
    }

    // fallback edges if any are empty (as in the JS)
    let top_fallback = if top_mod.is_empty() {
        vec![(0.0, 0.0), (100.0, 0.0)]
    } else {
        top_mod.clone()
    };
    let right_fallback = if right_mod.is_empty() {
        vec![(100.0, 0.0), (100.0, 100.0)]
    } else {
        right_mod.clone()
    };
    let bottom_fallback = if bottom_mod.is_empty() {
        vec![(0.0, 100.0), (100.0, 100.0)]
    } else {
        bottom_mod.clone()
    };
    let left_fallback = if left_mod.is_empty() {
        vec![(0.0, 0.0), (0.0, 100.0)]
    } else {
        left_mod.clone()
    };

    // widthRatio as in JS
    let width_ratio = ((width / 90.0) * 1000.0).floor() / 1000.0;

    // yWidth, xWidth baseline
    let mut y_width = 1.0;
    let mut x_width = if width.abs() > std::f64::EPSILON {
        (height / width).abs() * 1.0
    } else {
        1.0
    };

    // apply border_width param
    let (tbw, bbw, rbw, lbw) = match &props.border_width {
        BorderWidth::Single(v) => (*v, *v, *v, *v),
        BorderWidth::Four { t, b, r, l } => (*t, *b, *r, *l),
        BorderWidth::WidthHeight(w, h) => (*h, *h, *w, *w),
    };
    // if border_width is provided as single number, dials above will already be correct

    // compute final widths in percent-space as in original
    let top_width = width_ratio
        * (if props.edges.contains(&Edge::Top) {
            tbw
        } else {
            0.0
        });
    let bottom_width = width_ratio
        * (if props.edges.contains(&Edge::Bottom) {
            bbw
        } else {
            0.0
        });
    let right_width = width_ratio
        * (if props.edges.contains(&Edge::Right) {
            rbw
        } else {
            0.0
        });
    let left_width = width_ratio
        * (if props.edges.contains(&Edge::Left) {
            lbw
        } else {
            0.0
        });

    // variance function using noiseAlt2 ~ noise with "-alt2" suffix
    let variance = |w: f64, idx: usize, side: &str| -> f64 {
        let noise = seeded_noise(&format!("{}-alt2", props.seed), idx as i64, idx as i64);
        let variance_val = if side == "top" || side == "bottom" {
            props.width_variance.1
        } else {
            props.width_variance.0
        };
        if !props.edges.iter().any(|edge| edge.is(side)) {
            0.0
        } else {
            w + (0.5 + noise) * variance_val
        }
    };

    // Build inner paths by offsetting outer points
    let inner_left: Vec<(f64, f64)> = left_fallback
        .iter()
        .enumerate()
        .map(|(i, p)| (p.0 + variance(left_width, i, "left"), p.1))
        .collect();
    let inner_bottom: Vec<(f64, f64)> = bottom_fallback
        .iter()
        .enumerate()
        .map(|(i, p)| (p.0, p.1 - variance(bottom_width, i, "bottom")))
        .collect();
    let inner_right: Vec<(f64, f64)> = right_fallback
        .iter()
        .enumerate()
        .map(|(i, p)| (p.0 - variance(right_width, i, "right"), p.1))
        .collect();
    let inner_top: Vec<(f64, f64)> = top_fallback
        .iter()
        .enumerate()
        .map(|(i, p)| (p.0, p.1 + variance(top_width, i, "top")))
        .collect();

    PathResult {
        outer_top: top_fallback,
        outer_right: right_fallback,
        outer_bottom: bottom_fallback,
        outer_left: left_fallback,
        inner_top,
        inner_right,
        inner_bottom,
        inner_left,
    }
}

/// container for result paths
pub struct PathResult {
    pub outer_top: Vec<(f64, f64)>,
    pub outer_right: Vec<(f64, f64)>,
    pub outer_bottom: Vec<(f64, f64)>,
    pub outer_left: Vec<(f64, f64)>,
    pub inner_top: Vec<(f64, f64)>,
    pub inner_right: Vec<(f64, f64)>,
    pub inner_bottom: Vec<(f64, f64)>,
    pub inner_left: Vec<(f64, f64)>,
}

/// Convert a list of (x,y) -> "x% y%" and join with ", "
fn points_to_css(points: &[(f64, f64)]) -> String {
    points
        .iter()
        .map(|(x, y)| format!("{:.4}% {:.4}%", x, y))
        .collect::<Vec<_>>()
        .join(", ")
}

/// Equivalent of makePath(inner, outer) used in Vue — returns the combined polygon string
fn make_path_css(inner: &[(f64, f64)], outer: &[(f64, f64)]) -> String {
    let inner_rev = inner
        .iter()
        .rev()
        .map(|(x, y)| format!("{:.4}% {:.4}%", x, y))
        .collect::<Vec<_>>()
        .join(", ");
    let outer_s = outer
        .iter()
        .map(|(x, y)| format!("{:.4}% {:.4}%", x, y))
        .collect::<Vec<_>>()
        .join(", ");
    format!("{}, {}", inner_rev, outer_s)
}

/// Build the big content clip-path polygon string (combining inner top,right,bottom,left)
fn content_clip_path(pr: &PathResult) -> String {
    // order: inner.top, inner.right, reverse(inner.bottom), reverse(inner.left)
    let mut arr: Vec<String> = Vec::new();

    let top = pr
        .inner_top
        .iter()
        .map(|(x, y)| format!("{:.4}% {:.4}%", x, y))
        .collect::<Vec<_>>()
        .join(", ");
    let right = pr
        .inner_right
        .iter()
        .map(|(x, y)| format!("{:.4}% {:.4}%", x, y))
        .collect::<Vec<_>>()
        .join(", ");
    let mut bottom_rev = pr.inner_bottom.clone();
    bottom_rev.reverse();
    let bottom_s = bottom_rev
        .iter()
        .map(|(x, y)| format!("{:.4}% {:.4}%", x, y))
        .collect::<Vec<_>>()
        .join(", ");
    let mut left_rev = pr.inner_left.clone();
    left_rev.reverse();
    let left_s = left_rev
        .iter()
        .map(|(x, y)| format!("{:.4}% {:.4}%", x, y))
        .collect::<Vec<_>>()
        .join(", ");

    let concat = vec![top, right, bottom_s, left_s].join(", ");
    format!("polygon({})", concat)
}

/// The main function: produce the maud::Markup tree for the bordered component.
/// title and content are Markup (slots). props can be None to use defaults.
pub fn bordered_component(
    width: f64,
    height: f64,
    title: Option<Markup>,
    content: Markup,
    props: Option<Props>,
) -> Markup {
    let props = props.unwrap_or_default();

    // compute geometry
    let paths = create_path(width, height, &props);

    // polygon clip paths per edge
    let top_path = make_path_css(&paths.inner_top, &paths.outer_top);
    let bottom_path = make_path_css(&paths.inner_bottom, &paths.outer_bottom);
    let right_path = make_path_css(&paths.inner_right, &paths.outer_right);
    let left_path = make_path_css(&paths.inner_left, &paths.outer_left);

    // content clip path
    let content_path = content_clip_path(&paths);

    // filter id
    let filter_id = format!("goo-{}", props.id_card);

    // edges set for quick check
    let edges_set: HashSet<String> = props.edges.iter().map(|edge| edge.to_string()).collect();

    html! {
        div class="bordered" style={(format!("filter: url(#{})", filter_id))} {
            @if let Some(title) = title {
                h2 { (title) }
            }
            div class="content" style=(format!("clip-path: {};", &content_path)) {
                div class=(format!("content-inner max-w-min {}", props.content_classes)) style="filter: none; display: inline-block" {
                    (content)
                }
            }
            // borders overlay
            div class="borders" style=(format!("filter: url(#{}); pointer-events: none; position: absolute; height: 100%; width: 100%; top: 0; left: 0;", filter_id)) {
                // right
                @if edges_set.contains("right") {
                    div class="right" style=(format!("position:absolute; top:0; left:0; right:0; bottom:0; background: {}; clip-path: polygon({});", props.color, right_path)) {}
                } @else {
                    // hidden element preserved if you want structure
                    div class="right hidden" {}
                }

                // bottom
                @if edges_set.contains("bottom") {
                    div class="bottom" style=(format!("position:absolute; top:0; left:0; right:0; bottom:0; background: {}; clip-path: polygon({});", props.color, bottom_path)) {}
                } @else {
                    div class="bottom hidden" {}
                }

                // left
                @if edges_set.contains("left") {
                    div class="left" style=(format!("position:absolute; top:0; left:0; right:0; bottom:0; background: {}; clip-path: polygon({});", props.color, left_path)) {}
                } @else {
                    div class="left hidden" {}
                }

                // top
                @if edges_set.contains("top") {
                    div class="top" style=(format!("position:absolute; top:0; left:0; right:0; bottom:0; background: {}; clip-path: polygon({});", props.color, top_path)) {}
                } @else {
                    div class="top hidden" {}
                }
            }

            // hidden SVG (filter)
            // note: Maud will escape contents unless we insert raw; we can just emit plain svg elements
            svg style="visibility: hidden; position: absolute; width:0; height:0;" xmlns="http://www.w3.org/2000/svg" version="1.1" {
                defs {
                    (maud::DOCTYPE); // harmless placeholder; optional
                    // We build filter with goo=props.goo
                    (maud::PreEscaped(format!(
                        r#"<filter id="{id}">
                            <feGaussianBlur in="SourceGraphic" stdDeviation="{goo}" result="blur"/>
                            <feColorMatrix in="blur" type="matrix" values="1 0 0 0 0  0 1 0 0 0  0 0 1 0 0  0 0 0 19 -9" result="goo"/>
                            <feComposite in="SourceGraphic" in2="goo" operator="atop"/>
                        </filter>"#,
                        id=filter_id,
                        goo=props.goo
                    )))
                }
            }
        }
    }
}
