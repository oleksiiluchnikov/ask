<script>
    import { invoke } from "@tauri-apps/api/tauri";
    import { process } from "@tauri-apps/api";

    let content = "";

    const handleKeyDown = async (event) => {
        if (event.key === "Enter") {
            await invoke("process_input", { content });
            content = ""; // Clear the input field after processing
        } else if (event.key === "Escape") {
            await invoke("show_window");
            content = ""; // Clear the input field on escape
        }
    };
</script>

<input
    type="text"
    autoFocus
    bind:value={content}
    on:keydown={handleKeyDown}
    placeholder=""
/>
