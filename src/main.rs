use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        let args: Vec<String> = std::env::args().collect();

        if args.len() > 1 {
            let command = &args[1];

            if command == "--init" {
                println!("Inicializando emojify...");
                init();
            } else {
                println!("Comando no reconocido: {}", command);
            }
        } else {
            println!("Uso: emojify <comando>");
        }
    } else {
        emojify();
    }
}

fn emojify() {
    let options = &[
        "✨ feat",
        "🎉 new",
        "🐛 fix",
        "📗 docs",
        "🔧 chore",
        "🚀 perf",
        "🚧 wip",
        "🔥 remove",
        "💄style",
        "🔒 security",
        "🚚 move",
        "📦 package",
        "🔖 release",
        "🚑 hotfix",
        "🔨 refactor",
        "🔀 merge",
        "📄 license",
        "📝 test",
        "📝 ci",
        "📝 build",
        "📝 config",
        "📝 deploy",
        "📝 init",
        "📝 update",
        "📝 upgrade",
        "📝 downgrade",
        "📝 add",
        "📝 remove",
        "📝 fix",
        "📝 change",
        "📝 improve",
        "📝 optimize",
        "📝 clean",
        "📝 forma",
    ];
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
    let mut scopes = Input::with_theme(&binding);
    let scopes: String = scopes
        .with_prompt("Enter the scope of the commit")
        .with_prompt("Example: core, cli, docs, etc...")
        .interact()
        .unwrap();

    // Define the prompt and get user input for the commit message
    let mut message = Input::with_theme(&binding);
    let message: String = message
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
    let confirm = Confirm::with_theme(&binding)
        .with_prompt("Do you want to continue?")
        .default(true)
        .show_default(true)
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
        //close program
        std::process::exit(0x0100);
    }

    //confirm push
    let confirm = Confirm::with_theme(&binding)
        .with_prompt("Do you want to push?")
        .default(true)
        .show_default(true)
        .interact()
        .unwrap();
    if confirm {
        //know the branch default main
        let branch = std::process::Command::new("git")
            .arg("branch")
            .arg("--show-current")
            .output()
            .expect("failed to execute process");
        let branch = String::from_utf8_lossy(&branch.stdout);
        let branch = branch.trim();
        // Execute the git push command
        std::process::Command::new("git")
            .arg("push")
            .arg("origin")
            .arg(branch)
            .output()
            .expect("failed to execute process");

        println!("Pushed to {}", branch);
    } else {
        println!("Push canceled");
        //close program
        std::process::exit(0x0100);
    }
}

fn init() {
    std::process::Command::new("git")
        .arg("init")
        .output()
        .expect("error al inicializar el repositorio");

    std::process::Command::new("git")
        .arg("add")
        .arg(".")
        .output()
        .expect("error al agregar los archivos");

    std::process::Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg("🎉 (new) first commit")
        .output()
        .expect("error al hacer el primer commit");

    let remote: String = Input::new()
        .with_prompt("Ingrese la url del repositorio remoto")
        .interact()
        .unwrap();
    print!("{}", remote);

    if remote != "" {
        std::process::Command::new("git")
            .arg("remote")
            .arg("add")
            .arg("origin")
            .arg(remote)
            .output()
            .expect("error al agregar el repositorio remoto");
    }

    println!("Repositorio inicializado correctamente");
}
