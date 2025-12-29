interface Theme {
  id: string;
  name: string;
}

export function replacer(input: string, theme: Theme) {
  let output = input;
  output = output.replace(/\{\{THEME_NAME\}\}/g, theme.name);
  output = output.replace(/\{\{THEME_ID\}\}/g, theme.id);

  const currentTime = new Date();
  const year = currentTime.getFullYear()

  output = output.replace(/\{\{YEAR\}\}/g, `${year}`);

  return output;
}
