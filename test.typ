#import "@preview/polylux:0.4.0": *

#set page(paper: "presentation-16-9")

#set text(size: 25pt)

#slide[
    #polylux.pdfpc.speaker-note(
    ```md
    What if you could show code not just in an example, but in a video?

    Videos are useful in transfering lots of information, especially to provide context to code examples.
    ```
    )

    Code examples or code videos?
]

#slide[
    #pdfpc.speaker-note(
    ```md
    For example, this is example code for how to use the `transformrs` crate to talk to a large language model.
    
    As you can see, we load the `transformrs` crate and then request a chat completion.
    ```
    )

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

