# MDM

MDM is a CLI application designed to provide a definitive frame of reference for managing version-controlled documentation. Through its command set, it enforces a structured workflow that transforms Markdown into professional PDFs while maintaining a strict, predictable strategy for document integrity in shared, collaborative environments.

## Why MDM?

Throughout my academic career, I struggled to adopt Git and GitFlow for documentation. The core issue was a lack of a "frame of reference." While the workflow for source code was clear, applying those same rules to documentation was confusing.

### How "Not-To"

Early approaches I've made to the problem faced the following issues:

- Managing a large document as a single, massive file creates significant "technical debt". When multiple team members push changes, resolving merge commits becomes a nightmare. Ensuring the document remains coherent after several conflicting updates is time-consuming and prone to error.
- Splitting the document on many separate files manually is a solution I've personally tried with terrible results. Without a central authority, each team member ends up imposing their own criteria on how to name or split the document. This is perfectly fine with good communication, but its not a guarantee on every project or team.

### Why bother?

You might ask: “Why put yourself through this? Isn't the university just trying to be difficult?” I beg to differ. Despite the challenges, I have come to like the benefits of a Markdown-based documentation:

- **Single tool for everything:** Being able to write down the documentation from my IDE right after a coding task without a context switch that spaces me out.
- **Auditability:** A simple `git log` in the terminal tells you exactly who changed what and when.
- **Speed:** Markdown is significantly faster to write and format than the sluggish experience of a `.docx` file in Google Drive.
- **Automation:** Because LLMs output Markdown by default, your documentation is written on the same language AI uses. This allows your IDE-integrated AI agents to:
  - Automate Documentation Maintenance: Task an AI agent to update or generate documentation based on the specific changes in your latest Git commits. This ensures your docs stay in perfect sync with your code, making it an ideal workflow for Agile environments.
  - Bidirectional Context: Use your well-structured Markdown files as high-quality context for AI. By feeding your documentation strategy into an LLM, you enable the AI to output more accurate, context-aware code that respects your project's existing logic and structure.

## The workflow

### Structure

I've come around this issue with the idea of centralizing the structure of the document within the repository, on a file I named `schema.yaml`, which may be kept within the project's files.

The proposed project structure is the following.

```
my-project/            <-- Git Repository root
├── mdm/
│   └── schema.yaml    <-- The document's blueprint
├── sections/          <-- Individual markdown pieces
│   ├── architecture.md
│   ├── code_conventions.md
│   ├── project_goals.md
│   └── requirements.md
└── document.md        <-- The combined output
```

You can create said structure by using the following command, which will both initialize MDM and Git.

```sh
# Create the 'mdm' folder
mdm init

# Generate the sections folder based on 'schema.yaml'
mdm sync

# Combine all the files on the stablished order before a release
mdm build
```

### Strategy

The usage of said schema allows to work more easily on two different ways:

- **Enforced blueprint:** In this mode, you define a final, immutable structure in `schema.yaml` on the develop or main branch. This may suit cases like a company or university that enforces well-defined formats that can't or shouldn't be changed.
- **Collaborative canvas:** In this mode, you can allow contributors to propose structural changes through Pull Requests. When a team member wants to add, rename, or reorder a section, they simply modify the `schema.yaml`. The PR serves as a clear as a communication channel.

As you can see, both conditions are fully Git and GitFlow compatible.
- Changes to the document are made by pushing into a feature branch (feature = new section).
- You can prevent certain changes by using Branch Protection Rules.
- Pull Requests (PRs) become your primary editorial tool.

## Installation

MDM is distributed via **GitHub Releases**. You can download pre-built binaries for Windows, macOS, and Linux without needing to install Rust or compile the code manually.

### Dependencies

#### git (Mandatory):

MDM uses git to manage document versions and history integrity collaboratively. Certain CLI commands directly calls certain git commands as subprocesses. Get it at [Official Website](https://git-scm.com).

#### md-to-pdf (Recommended)

Markdown files dont directly have the visual appeal that `.docx` files may have, so you may want to have a way to turn them into stylish `.pdf` files. Get it at [NPM Website](https://www.npmjs.com/package/md-to-pdf).

# A couple notes from the Developer

## Disclaimer

- **I'm a rookie:** This is my first serious project. I have used this project as an excuse to learn Rust, which I'm growing to like because of the ecosystem and features.
- **Built for me:** This is a tool I made for myself. While I plan to add features I personally need, the core contribution of this project is sharing my personal approach on how to manage documentation.
- **Target audience:** My main target audiance are Software Engineering students at my university. Most likely no one other than them end up using this tool, but I'm proud enough about it to share it on a public so anyone who finds it useful can use it too.

## Current status

- **Incomplete feature:** I had initially planned to release this with a variable feature, which I haven't removed despite not finishing. This one is certain to be the next feature on the next release.
- **schema.yaml:** upon using `mdm init` the default `schema.yaml` self-contains an example and demonstration of all the features and options it supports.
