// @generated automatically by Diesel CLI.

diesel::table! {
    albums (id) {
        id -> Text,
        title -> Text,
        project_id -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        artwork -> Nullable<Text>,
        description -> Nullable<Text>,
        links -> Nullable<Jsonb>,
    }
}

diesel::table! {
    dates (id) {
        id -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        event_id -> Nullable<Text>,
        start_date -> Timestamp,
        end_date -> Timestamp,
        venue -> Text,
        city -> Text,
        state -> Nullable<Text>,
        country -> Text,
        ticket_url -> Nullable<Text>,
        venue_url -> Nullable<Text>,
        title -> Text,
        slug -> Text,
        content -> Text,
        project_id -> Text,
    }
}

diesel::table! {
    email_campaigns (id) {
        id -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        project_id -> Text,
        subject -> Text,
        content -> Text,
    }
}

diesel::table! {
    email_deliveries (id) {
        id -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        campaign_id -> Text,
        subscriber_id -> Text,
        status -> Text,
    }
}

diesel::table! {
    events (id) {
        id -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    files (id) {
        id -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        name -> Text,
        #[sql_name = "type"]
        type_ -> Text,
        size -> Int4,
        url -> Text,
        folder_id -> Text,
        project_id -> Text,
    }
}

diesel::table! {
    folders (id) {
        id -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        name -> Text,
        project_id -> Text,
        parent_id -> Nullable<Text>,
    }
}

diesel::table! {
    menu_item_labels (id) {
        id -> Text,
        menu_item_id -> Text,
        language -> Text,
        label -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    menu_items (id) {
        id -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        menu_id -> Text,
        parent_id -> Nullable<Text>,
        #[sql_name = "type"]
        type_ -> Text,
        order -> Int4,
        page_id -> Nullable<Text>,
        submenu_id -> Nullable<Text>,
        url -> Nullable<Text>,
        active -> Int4,
    }
}

diesel::table! {
    menus (id) {
        id -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        name -> Text,
        description -> Nullable<Text>,
        active -> Int4,
        website_id -> Text,
    }
}

diesel::table! {
    page_content (id) {
        id -> Uuid,
        page_id -> Text,
        default_content -> Bool,
        preview -> Bool,
        language -> Text,
        content -> Jsonb,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        template -> Nullable<Text>,
    }
}

diesel::table! {
    pages (id) {
        id -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        title -> Text,
        slug -> Text,
        #[sql_name = "type"]
        type_ -> Text,
        options -> Jsonb,
        active -> Int4,
        home -> Bool,
        website_id -> Text,
    }
}

diesel::table! {
    payment_invoices (id) {
        id -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        subscription_id -> Text,
        stripe_invoice_id -> Text,
        amount_cents -> Int4,
        status -> Text,
        paid_at -> Nullable<Timestamp>,
        payment_intent_id -> Nullable<Text>,
    }
}

diesel::table! {
    payment_plans (id) {
        id -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        name -> Text,
        description -> Nullable<Text>,
        base_price_cents -> Int4,
        per_site_price_cents -> Int4,
        promotional_code -> Nullable<Text>,
        promotional_discount_percent -> Nullable<Int4>,
        active -> Int4,
    }
}

diesel::table! {
    payment_subscriptions (id) {
        id -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        project_id -> Text,
        plan_id -> Text,
        stripe_subscription_id -> Text,
        status -> Text,
        current_period_start -> Timestamp,
        current_period_end -> Timestamp,
        active_sites_count -> Int4,
    }
}

diesel::table! {
    project_extras (id) {
        id -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        project_id -> Text,
        extra -> Text,
        active -> Int4,
    }
}

diesel::table! {
    project_promotions (id) {
        id -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        project_id -> Text,
        discount_percent -> Int4,
        start_date -> Timestamp,
        end_date -> Nullable<Timestamp>,
        description -> Nullable<Text>,
        active -> Int4,
    }
}

diesel::table! {
    project_users (id) {
        id -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        user_id -> Text,
        project_id -> Text,
        role -> Text,
        terms_date -> Nullable<Timestamp>,
    }
}

diesel::table! {
    projects (id) {
        id -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        name -> Nullable<Text>,
        #[sql_name = "type"]
        type_ -> Text,
        active -> Int4,
        #[max_length = 10]
        default_language -> Varchar,
    }
}

diesel::table! {
    route_records (id) {
        id -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        domain -> Text,
        subdomain -> Nullable<Text>,
        website_id -> Text,
    }
}

diesel::table! {
    stripe_customers (id) {
        id -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        user_id -> Text,
        stripe_customer_id -> Text,
    }
}

diesel::table! {
    subscribers (id) {
        id -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        email -> Text,
        name -> Nullable<Text>,
        city -> Nullable<Text>,
        state -> Nullable<Text>,
        country -> Nullable<Text>,
    }
}

diesel::table! {
    subscriptions (id) {
        id -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        subscriber_id -> Text,
        project_id -> Text,
        subscription_method -> Nullable<Text>,
        unsubscribed_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    user_roles (role_name) {
        role_name -> Text,
        value -> Int4,
    }
}

diesel::table! {
    user_sessions (id) {
        id -> Text,
        created_at -> Timestamp,
        expires_at -> Timestamp,
        user_id -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        email -> Text,
        password -> Text,
        name -> Nullable<Text>,
        role -> Text,
        terms_date -> Nullable<Timestamp>,
        confirmed_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    websites (id) {
        id -> Text,
        project_id -> Text,
        theme -> Text,
        theme_config -> Nullable<Jsonb>,
        active -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        title -> Varchar,
    }
}

diesel::joinable!(albums -> projects (project_id));
diesel::joinable!(dates -> events (event_id));
diesel::joinable!(dates -> projects (project_id));
diesel::joinable!(email_campaigns -> projects (project_id));
diesel::joinable!(email_deliveries -> email_campaigns (campaign_id));
diesel::joinable!(email_deliveries -> subscribers (subscriber_id));
diesel::joinable!(files -> folders (folder_id));
diesel::joinable!(files -> projects (project_id));
diesel::joinable!(folders -> projects (project_id));
diesel::joinable!(menu_item_labels -> menu_items (menu_item_id));
diesel::joinable!(menu_items -> pages (page_id));
diesel::joinable!(menus -> websites (website_id));
diesel::joinable!(page_content -> pages (page_id));
diesel::joinable!(pages -> websites (website_id));
diesel::joinable!(payment_invoices -> payment_subscriptions (subscription_id));
diesel::joinable!(payment_subscriptions -> payment_plans (plan_id));
diesel::joinable!(payment_subscriptions -> projects (project_id));
diesel::joinable!(project_extras -> projects (project_id));
diesel::joinable!(project_promotions -> projects (project_id));
diesel::joinable!(project_users -> projects (project_id));
diesel::joinable!(project_users -> users (user_id));
diesel::joinable!(route_records -> websites (website_id));
diesel::joinable!(stripe_customers -> users (user_id));
diesel::joinable!(subscriptions -> projects (project_id));
diesel::joinable!(subscriptions -> subscribers (subscriber_id));
diesel::joinable!(user_sessions -> users (user_id));
diesel::joinable!(websites -> projects (project_id));

diesel::allow_tables_to_appear_in_same_query!(
    albums,
    dates,
    email_campaigns,
    email_deliveries,
    events,
    files,
    folders,
    menu_item_labels,
    menu_items,
    menus,
    page_content,
    pages,
    payment_invoices,
    payment_plans,
    payment_subscriptions,
    project_extras,
    project_promotions,
    project_users,
    projects,
    route_records,
    stripe_customers,
    subscribers,
    subscriptions,
    user_roles,
    user_sessions,
    users,
    websites,
);
