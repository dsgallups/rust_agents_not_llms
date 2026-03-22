# Example of connecting a bunch of things together using a management system

Steps to get here
# Init repo
`cargo new rust_agents_not_llms`
# Add dependencies
- `tokio`: an async runtime. Basically the de-facto runtime for multithreaded async executors (the full feature gives me access to all the things tokio can do. You can constrain this to be like i/o, a single-threaded executor if needed, etc.)
