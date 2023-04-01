use dialoguer::{theme::ColorfulTheme, Select};


   

fn main() {
    let options = &["✨ feat", "🎉 new", "🐛 fix", "📗 docs", "🔧 chore", "🚀 perf", "🚧 wip", "🔥 remove", "💄style", "🔒 security", "🚚 move", "📦 package", "🔖 release", "🚑 hotfix", "🔨 refactor", "🔀 merge", "📄 license", "📝 test", "📝 ci", "📝 build", "📝 config", "📝 deploy", "📝 init", "📝 update", "📝 upgrade", "📝 downgrade", "📝 add", "📝 remove", "📝 fix", "📝 change", "📝 improve", "📝 optimize", "📝 clean", "📝 forma"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select an option for commit")
        .items(options)
        .default(0)
        .interact()
        .unwrap();
    //input scope
    let binding = ColorfulTheme::default();
    let mut scopes = dialoguer::Input::with_theme(&binding);
    let scopes: String = scopes
        .with_prompt("Enter the scope of the commit")
        .with_prompt("Example: core, cli, docs, etc...")
        .interact()
        .unwrap();
    let mut message = dialoguer::Input::with_theme(&binding);
    let message : String= message
        .with_prompt("Enter the message of the commit")
        .interact()
        .unwrap();
    //message would be uppercase first letter
    let message = message
        .chars()
        .enumerate()
        .map(|(i, c)| {
            if i == 0 {
                c.to_uppercase().to_string()
            } else {
                c.to_string()
            }
        })
        .collect::<String>();

    //print the commit first selection [scope] message
    println!("{} ({}) {}", options[selection], scopes, message);
    //execute the command git commit -m "message"
    std::process::Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(format!("{} ({}) {}", options[selection], scopes, message))
        .output()
        .expect("failed to execute process");
    }
