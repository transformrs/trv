#import "@preview/polylux:0.4.0": *

#set page(paper: "presentation-16-9")

#set text(size: 25pt)

#slide[
    #toolbox.pdfpc.speaker-note(
    ```md
    
    What if you could show code not just in text, but in a video?

    Videos are good for transfering lots of information.
    ```
    )

    \
    #align(center)[Code examples or code videos?]
]

#slide[
    #toolbox.pdfpc.speaker-note(
    ```md
    For example, this is code for the `transformrs`-crate.
    
    As you can see, we load the crate and then request a chat completion.
    ```
    )

    #set text(size: 20pt)

    ```rust
    use transformrs::chat;
    use transformrs::Message;
    use transformrs::Provider;

    #[tokio::main]
    async fn main() {
        // ...
        let resp = chat::chat_completion(&provider, &key, model, &messages)
            .await
            .unwrap()
            .structured()
            .unwrap();
        println!("{}", resp.choices[0].message.content);
    }
    ```
]

