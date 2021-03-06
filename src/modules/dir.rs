use dirs;
use icons;
use std::env;
use std::path::Path;
use Segment;

pub fn segment(segment: &mut Segment, _: &[&str]) {
    let mut wd = env::current_dir().unwrap();

    // processes aliases
    let mut aliases = match env::var("SILVER_DIR_ALIASES") {
        Ok(var) => var
            .split(':')
            .map(|alias| alias.to_owned())
            .collect::<Vec<String>>(),
        Err(_) => vec![],
    };
    if aliases.len() % 2 != 0 {
        panic!("invalid $SILVER_DIR_ALIASES");
    }
    // default home alias
    if let Some(home) = dirs::home_dir() {
        aliases.push(home.to_str().unwrap().to_owned());
        aliases.push(icons::get("home"));
    }
    for i in 0..aliases.len() / 2 {
        let dir = &aliases[i * 2];
        let icon = &aliases[i * 2 + 1];
        wd = match wd.strip_prefix(dir) {
            Ok(stripped) => Path::new(icon).join(stripped.to_path_buf()),
            Err(_) => wd.clone(),
        }
    }

    // processes length
    if let Ok(len) = env::var("SILVER_DIR_LENGTH") {
        let len = len.parse::<usize>().expect("invalid $SILVER_DIR_LENGTH");
        let iter_len = wd.iter().count();
        let mut i = 0;
        wd = wd
            .iter()
            .map(|component| {
                i += 1;
                if i != iter_len && component.len() > len {
                    component.to_str().unwrap().chars().take(len).collect()
                } else {
                    component.to_str().unwrap().to_owned()
                }
            })
            .collect();
    }

    segment.value = wd.to_str().unwrap().to_owned();

    if segment.value != "/" && segment.value.ends_with('/') {
        segment.value.pop();
    }
}
