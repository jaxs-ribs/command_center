use crate::kinode::process::llm::groq_chat;
use kinode_process_lib::println;

const MAX_POSTS: usize = 20;

pub fn filter_posts(
    rules: Vec<String>,
    post_contents: Vec<String>,
) -> Result<Vec<bool>, String> {
    if post_contents.len() > MAX_POSTS {
        return Err(format!(
            "Too many posts. Maximum allowed is {}, but {} were provided.",
            MAX_POSTS,
            post_contents.len()
        ));
    }

    let post_contents_len = post_contents.len();
    let base_prompt = base_prompt(rules, post_contents);

    let res = match groq_chat(&base_prompt, Some("llama-3.1-70b-versatile")) {
        Ok(res) => res,
        Err(e) => return Err(format!("Error in the groq chat: {}", e)),
    };

    println!("Parsing Groq chat response");
    let parsed_result: Vec<bool> = res
        .trim()
        .chars()
        .map(|c| match c {
            '1' => true,
            '0' => false,
            _ => {
                println!("Warning: Unexpected character '{}' in response", c);
                false
            }
        })
        .collect();

    if parsed_result.len() != post_contents_len {
        return Err(format!(
            "Mismatch between number of posts and parsed results"
        ));
    }

    Ok(parsed_result)
}

fn base_prompt(rules: Vec<String>, post_contents: Vec<String>) -> String {
    let rules_str = rules
        .iter()
        .enumerate()
        .map(|(i, rule)| format!("{}. {}\n", i + 1, rule))
        .collect::<String>();

    let posts_str = post_contents
        .iter()
        .map(|post| format!("\n\n---\n{}\n---\n\n", post))
        .collect::<String>();

    format!(
        r###"
For each of the following posts, you will answer with 0 or 1. 0 if the answer is no, 1 if the answer is yes.
The answer is 0 if any of the rules are violated, otherwise it is 1.

The rules are:
{}

------------
The answer should just be a string of 1s and 0s representing the answer to the corresponding post, and nothing else. This is because it will get parsed to a system, so please only answer with 0s and 1s, and make sure the amount of chars matches the amount of posts, and the order is preserved. 

Here are the posts, demarcated by "---":

{}
"###,
        rules_str, posts_str
    )
}
