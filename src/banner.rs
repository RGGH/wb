use termion::color;

pub fn owl() -> std::io::Result<()> {
    println!("Server listening on port 8080");
    let banner = format!(
        r#"   __________
  / ___  ___ \
 / / {red}@{reset} \/ {red}@{reset} \ \
 \ \___/\___/ /\
  \____\/____/||
 /     /\\\\\//
|     |\\\\\\
 \      \\\\\\
   \______/\\\\
    _||_||_"#,
        red = color::Fg(color::Red),
        reset = color::Fg(color::Reset)
    );

    println!("{}", banner);

    Ok(())
}
