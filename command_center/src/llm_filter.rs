use crate::kinode::process::llm::groq_chat;
use kinode_process_lib::println;

const MAX_POSTS: usize = 100;

pub fn filter_posts(rules: Vec<String>, post_contents: Vec<String>) -> Result<Vec<bool>, String> {
    if post_contents.len() > MAX_POSTS {
        return Err(format!(
            "Too many posts. Maximum allowed is {}, but {} were provided.",
            MAX_POSTS,
            post_contents.len()
        ));
    }

    let post_contents_len = post_contents.len();
    let base_prompt = base_prompt(rules, post_contents);

    let res = match groq_chat(&base_prompt, Some("llama3-groq-70b-8192-tool-use-preview")) {
    // let res = match groq_chat(&base_prompt, Some("llama-3.1-70b-versatile")) {
        Ok(res) => res,
        Err(e) => return Err(format!("Error in the groq chat: {}", e)),
    };

    println!("The total amount of posts is: {}", post_contents_len);

    println!("Groq chat response: {}", res);
    println!("The length of the response is: {}", res.len());

    // Strip the response of anything that isn't 0 or 1
    let res: String = res.chars().filter(|&c| c == '0' || c == '1').collect();

    println!("Filtered Groq chat response: {}", res);

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
        .enumerate()
        .map(|(i, post)| format!("Post #{}: {}\n---\n", i + 1, post))
        .collect::<String>();

    format!(
        r###"You are a content moderator tasked with evaluating posts based on a set of rules. Your job is to output a string of 0s and 1s, where each digit corresponds to a post's compliance with all rules.

Rules for evaluation:
{}
Evaluation instructions:
1. Read each post carefully.
2. Check if the post violates ANY of the rules.
3. Output 1 if the post complies with ALL rules, 0 if it violates ANY rule.

Important:
- Your response must ONLY contain 0s and 1s, with NO spaces, punctuation, or other characters.
- The number of digits in your response MUST EXACTLY MATCH the number of posts ({}).
- Maintain the order: the first digit corresponds to Post #1, the second to Post #2, and so on.

Example output for 5 posts: 10110

Posts to evaluate:
{}
Your evaluation (exactly {} digits of 0s and 1s):
"###,
        rules_str,
        post_contents.len(),
        posts_str,
        post_contents.len()
    )
}
