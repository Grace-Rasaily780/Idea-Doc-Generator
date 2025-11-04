# Idea Doc Creator

`idea_doc_creator` is a CLI tool that helps you quickly document your ideas using a structured template. It's designed to make the process of capturing and evaluating ideas as seamless as possible.

## Problem Solved

The primary problem this tool addresses is the time-consuming and repetitive nature of manually creating daily idea documentation. By automating this, it saves time and makes the process feel less like work, allowing for a more streamlined and enjoyable experience.

## Features

-   **Interactive CLI:** Guides you through the idea documentation process with a series of questions.
-   **Structured Output:** Generates a well-formatted Markdown file with your idea's details.
-   **Feasibility & Impact Scoring:** Helps you evaluate your ideas with a simple scoring system.
-   **Automated File Naming:** Creates a new file for each day with the format `YYYY_MM_DD.md`.
-   **Safe Interruption:** If you interrupt the process (e.g., with `Ctrl+C`), it cleans up by removing the incomplete file.

## Installation

You can install `idea_doc_creator` from this repository:

```bash 
git clone https://github.com/Grace-Rasaily780/Idea-Doc-Generator
cargo build --release
cd target/release
./idea_doc_creator
```

## Usage

To start documenting a new idea, simply run:

```bash
idea_doc_creator
```

The tool will then guide you through the following steps:

1.  **Idea Prompt:** Choose from a list of prompts to get you started.
2.  **Raw Idea:** Write down your initial thoughts without any filtering.
3.  **Idea Breakdown:** Flesh out the core concepts of your idea.
4.  **Feasibility & Impact Scoring:** Score your idea based on several criteria.

Once you've completed the steps, a new Markdown file will be saved in the current directory.

## Contributing

Contributions are welcome! If you have any ideas for improvements or new features, feel free to open an issue or submit a pull request.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
