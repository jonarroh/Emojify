use dialoguer::{theme::ColorfulTheme, Select};

fn main() {
    // Define an array of commit options
    let options = &[
        "✨ feat", "🎉 new", "🐛 fix", "📗 docs",
        "🔧 chore", "🚀 perf", "🚧 wip", "🔥 remove",
        "💄style", "🔒 security", "🚚 move", "📦 package",
        "🔖 release", "🚑 hotfix", "🔨 refactor", "🔀 merge",
        "📄 license", "📝 test", "📝 ci", "📝 build",
        "📝 config", "📝 deploy", "📝 init", "📝 update",
        "📝 upgrade", "📝 downgrade", "📝 add", "📝 remove",
        "📝 fix", "📝 change", "📝 improve", "📝 optimize", "📝 clean", "📝 forma"];
    // Use dialoguer to prompt the user for input
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select an option for commit")
        .items(options)
        .default(0)
        .interact()
        .unwrap();

    // Define the theme for the prompts
    let binding = ColorfulTheme::default();

    // Define the prompt and get user input for the commit scope
    let mut scopes = dialoguer::Input::with_theme(&binding);
    let scopes: String = scopes
        .with_prompt("Enter the scope of the commit")
        .with_prompt("Example: core, cli, docs, etc...")
        .interact()
        .unwrap();

    // Define the prompt and get user input for the commit message
    let mut message = dialoguer::Input::with_theme(&binding);
    let message : String= message
        .with_prompt("Enter the message of the commit")
        .interact()
        .unwrap();

    // Capitalize the first letter of the commit message
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

    // Print the user's input for the commit and execute the commit command
    println!("The commit will be: ");
    println!("{} ({}) {}", options[selection], scopes, message);

    //Option to continue or cancel the commit
    let confirm = dialoguer::Confirm::with_theme(&binding)
        .with_prompt("Do you want to continue?")
        .interact()
        .unwrap();

    // If the user confirms the commit, execute the git commit command
    if confirm {
    // Execute the git commit command
    std::process::Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(format!("{} ({}) {}", options[selection], scopes, message))
        .output()
        .expect("failed to execute process");
    } else {
        println!("Commit canceled");
    }
}
