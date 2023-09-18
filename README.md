# Github tracker

## Description

Given that I have zero trust in github, as should you, given that they can do DMCA takedowns with no warning, I have decided to create this little rust software to keep track of my github repositories and locally save my issues and pull-request data. It's not perfect, but Some() is better than None.

## Usage

Run:

```
cargo run -- --help
```

to see the available options.

## How it works

The software downloads all the issues to json files (issue outline + comments). When the program is invoked repeatedly, it'll just check if anything has changed in the issues. If I change occurs, the data will be downloaded again for that particular issue. Simple and easy. Nothing complicated.
