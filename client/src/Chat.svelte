<script>
    import { onMount } from "svelte";
    let messages = [{ text: "Hi, ask me anything!", user: false }];
    let input = "";
    let error = "";

    async function getGptResponse(chatData) {
        try {
            let response = await fetch("/api/chat", {
                method: "POST",
                headers: { "Content-Type": "application/json" },
                body: JSON.stringify(chatData),
            });

            if (!response.ok) {
                throw new Error(`HTTP error! status: ${response.status}`);
            }
            let data = await response.json();
            console.log(data);
            return data;
        } catch (err) {
            error = err.message;
            console.error("Error occurred during fetch:", err);
        }
    }

    async function addMessage() {
        if (!input) return;
        messages = [...messages, { text: input, user: true }];
        let response = await getGptResponse({ messages });
        input = "";
        if (response) {
            messages = [...messages, { text: response, user: false }];
        } else {
            console.error(
                "Error occurred while getting GPT response. Error: ",
                error
            );
        }
    }
</script>

<h2>Chat with ChatGPT!</h2>
<div id="chat-window">
    {#each messages as message (message)}
        <div class={message.user ? "user-message" : "gpt-message"}>
            {message.text}
        </div>
    {/each}
</div>
<form on:submit|preventDefault={addMessage} class="prompt">
    <input bind:value={input} placeholder="Type a message..." />
    <button type="submit">Send</button>
</form>

<style>
    body {
        font-family: var(--font-primary);
        background-color: var(--background-color);
        color: var(--text-color);
    }

    #chat-window {
        height: 70vh; /* Adjusted for a fixed height */
        overflow: auto;
        border: 1px solid #333;
        padding: 15px;
        margin: 20px auto;
        background-color: var(--secondary-color);
        border-radius: var(--border-radius);
        box-shadow: var(--box-shadow);
        width: 90%;
        max-width: 800px;
    }

    .prompt {
        max-width: 800px;
        width: 90%;
        display: flex;
        margin: 10px auto;
        border-radius: var(--border-radius);
        overflow: hidden; /* Contain the child elements */
    }

    input {
        flex-grow: 1;
        border: none;
        padding: 10px;
        border-right: 1px solid #333;
        border-radius: var(--border-radius) 0 0 var(--border-radius);
        background-color: #333;
        color: var(--text-color);
        box-sizing: border-box;
    }

    button {
        border: none;
        background-color: var(--primary-color);
        color: white;
        padding: 10px 16px;
        border-radius: 0 var(--border-radius) var(--border-radius) 0;
        cursor: pointer;
        box-sizing: border-box;
    }

    .user-message,
    .gpt-message {
        margin: 10px;
        padding: 10px;
        border-radius: var(--border-radius);
        color: var(--text-color);
    }

    .user-message {
        background-color: rgba(58, 58, 58, 0.9);
        align-self: flex-end;
        border: 1px solid #444;
        color: white;
    }

    .gpt-message {
        background-color: rgba(42, 42, 42, 0.9);
        border: 1px solid #444;
    }

    h2 {
        text-align: center;
        margin-top: 20px;
        color: var(--text-color);
    }
</style>
