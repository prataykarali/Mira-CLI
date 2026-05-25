<img src="https://cdn.prod.website-files.com/677c400686e724409a5a7409/6790ad949cf622dc8dcd9fe4_nextwork-logo-leather.svg" alt="NextWork" width="300" />

# Build a CLI Memory Companion in Rust

**Project Link:** [View Project](https://learn.nextwork.org/projects/1b5596de-2ea9-4c51-b690-4244ef050ed7)

**Author:** Pratay Karali  
**Email:** Pratay.karali2005@gmail.com

---

![Image](https://learn.nextwork.org/proud_white_zealous_hyena/uploads/1b5596de-2ea9-4c51-b690-4244ef050ed7_s3xgm5x6)

## Building a CLI Memory Companion in Rust

### Project goals and motivation

In this project, I'm building Rust CLI companion with chat, memory and forget commands ,so that I can learn sqlite based queries, and rusqlite Db. Learn how to integrate this into a CLI for a RAG system.

## Setting Up the Rust Project

### Project scaffolding and dependencies

In this step, I'm setting up rust toolchain,scaffold a new binary file and add rusqlite and clap dependencies so that I can pull both crates which power Mira. for talking to database and parse terminal commands.

![Image](https://learn.nextwork.org/proud_white_zealous_hyena/uploads/1b5596de-2ea9-4c51-b690-4244ef050ed7_p9o6d5ie)

### Key crates: rusqlite and clap

I added rusqlite which handles connecting to and interacting with a SQLite database, and clap which handles parsing the subcommands I type in the terminal so Mira can understand my CLI input.

## Designing the CLI Interface with Clap

### Defining subcommands with derive macros

In this step, I'm defining the CLI struct and commands so that Mira can use 3 tools for chatting,recall memories and forgetting memories . This gives Mira a way to understand what the user is asking for and act like a companion ! 

![Image](https://learn.nextwork.org/proud_white_zealous_hyena/uploads/1b5596de-2ea9-4c51-b690-4244ef050ed7_lnucv7g2)

### Understanding the Commands enum

The three variants represent print a message which user provides in the format chat: Hello, the memory is printed when memory command is passed and forget when forget command is passed

## Creating the SQLite Database and Schema

### Opening a connection with WAL mode

In this step, I'm setting up a local sqLite database so that Mira can have memory. After the database integration she'll ne able to remember the user and every time Mira starts , she opens this database and makes sure her tables exists.

![Image](https://learn.nextwork.org/proud_white_zealous_hyena/uploads/1b5596de-2ea9-4c51-b690-4244ef050ed7_jbevbfun)

### Episodes and facts table design

I created a table called episodes which stores every message with timestamp and a table called facts which stores user facts with a key value pair.

## Implementing Chat with Fact Extraction and UPSERT

### Inserting messages and generating responses

In this step, I'm building core chat logic so that Mira can log it,scan it and respond with something relevant!

### Pattern matching and INSERT OR REPLACE

When I say something like "my name is Alex", Mira detects the pattern by lowercasing the message and checking for known prefixes like "my name is " or "i live in " using the extract_after helper function. It grabs the value after the prefix (up to the first punctuation mark), and stores it by calling upsert_fact, which runs an INSERT OR REPLACE into the facts table with a key like "user_name" and the extracted value. If the key already exists, it overwrites the old value.

![Image](https://learn.nextwork.org/proud_white_zealous_hyena/uploads/1b5596de-2ea9-4c51-b690-4244ef050ed7_sqtdakzx)

## Querying Memory and Wiping the Database

### Retrieving and clearing stored data

In this step, I'm setting up handling function for memory and forget so that I can see what facts Mira remembers and what she forgets.

![Image](https://learn.nextwork.org/proud_white_zealous_hyena/uploads/1b5596de-2ea9-4c51-b690-4244ef050ed7_ta5nigqo)

### DELETE vs DROP TABLE in handle_forget

The handle_forget function uses DELETE FROM episodes and DELETE FROM facts to clear all data. It uses DELETE instead of DROP TABLE because DELETE FROM removes all rows but keeps the table structure intact, so Mira can start collecting new memories immediately after a forget without needing to recreate the tables.

## Secret Mission: Mira's Identity Card

### Mapping database keys to human-readable labels

I mapped keys to labels by creating a label_map vec of tuples that pairs each internal database key (like "user_name") with a human-readable label (like "Name"). Then, for each fact returned from the database, I used .iter().find() to look up the matching label. If no match is found in the map, it falls back to displaying the raw key string using unwrap_or(key.as_str())

## Reflections and Key Takeaways

### Tools and concepts mastered

The key tools I learnt were clap for building a CLI with subcommands using Rust's derive macro, and rusqlite for creating and managing a SQLite database — including opening connections, inserting rows, querying data, and deleting records. Key concepts I learnt include UPSERT logic using INSERT OR REPLACE to keep facts up to date without duplicates, pattern-based fact extraction from user messages using string matching, and structuring a Rust CLI app with separate handler functions for each subcommand (chat, memory, forget)

### Time and challenges

This project took me approximately 2hrs. The most challenging part was adding the logic for chat, memory, forget and who

I did this project today to learn how to create a keyword matching RAG system. Another skill I want to learn is Vector searching and LLM RAG systems.

---

*Built with [NextWork](https://learn.nextwork.org) - [View this project](https://learn.nextwork.org/projects/1b5596de-2ea9-4c51-b690-4244ef050ed7)*
