<script lang="ts">
    import { ask } from "@tauri-apps/api/dialog";
    import { open } from "@tauri-apps/api/dialog";
    import { message } from "@tauri-apps/api/dialog";
    import { invoke } from "@tauri-apps/api/tauri";
    import toast from "svelte-french-toast";

    // import { appDir } from "@tauri-apps/api/path";

    let options = {};

    let directories: string[] = [];
    let files: string[] = [];
    // async function handleUploadStuff(directory: boolean, multiple: boolean) {
    //     // Open a selection dialog for directories
    //     const selected = await open({
    //         directory: true,
    //         multiple: false,
    //     });
    //     if (selected == null) {
    //         console.log("nothing selected");
    //         await message("No directory selected", {
    //             title: "Tauri",
    //             type: "error",
    //         });
    //     } else {
    //         if (Array.isArray(selected)) {
    //             selected.forEach((file) => {
    //                 if (files.includes(file) || directories.includes(file)) {
    //                     toast(
    //                         `${
    //                             directory ? "Directory" : "File"
    //                         } already exists! ${file}`,
    //                         {
    //                             icon: "👏",
    //                             // duration: 3000,
    //                         }
    //                     );
    //                 } else {
    //                     if (directory) {
    //                         directories = [file, ...directories];
    //                     } else {
    //                         files = [file, ...files];
    //                     }
    //                 }
    //             });
    //             console.log("Got an array of files: ", selected);
    //         } else {
    //             // directories = [];
    //             toast(`Unknown shit happened. ${selected}`, {
    //                 duration: 6900,
    //             });
    //         }
    //     }
    // }

    async function handleUploadDirectories() {
        // keep it simple
        // Open a selection dialog for directories
        const selected = await open({
            directory: true,
            multiple: true,
            recursive: true,
            title: "Select one or more directory to upload",
        });
        if (Array.isArray(selected)) {
            selected.forEach((file) => {
                if (!directories.includes(file)) {
                    directories = [file, ...directories];
                } else {
                    toast("Directory already exists so ignoring it", {
                        icon: "⚠️",
                    });
                }
            });
            // user selected multiple directories
        } else if (selected === null) {
            // user cancelled the selection
        } else {
            // things will be handled from here if `multiple` is false
            // we don't care it here
            toast(`single: ${selected}`, {
                duration: 5000,
            });
            // user selected a single directory
        }
    }
    async function handleUploadFiles() {
        // keep it simple
        // Open a selection dialog for directories
        const selected = await open({
            multiple: true,
            title: "Select one or more file to upload",
            // multiple: true,
        });
        if (Array.isArray(selected)) {
            selected.forEach(async (file) => {
                if (!files.includes(file)) {
                    console.log(file);
                    const isFile = await invoke("check_file_or_directory", {
                        path: file,
                    });
                    if (isFile == 0) {
                        files = [file, ...files];
                    } else {
                        toast(`${file} is not a file, is a ${isFile}`);
                    }
                } else {
                    toast("File already exists so ignoring it", {
                        icon: "⚠️",
                    });
                }
            });
            // user selected multiple directories
        } else if (selected === null) {
            // user cancelled the selection
        } else {
            // things will be handled from here if `multiple` is false
            // we don't care it here
            toast(`single: ${selected}`, {
                duration: 5000,
            });
            // user selected a single directory
        }
    }
</script>

<button on:click={handleUploadDirectories}>
    Upload Directory / Directories</button
>
<button on:click={handleUploadFiles}> Upload File / Files</button>

<div>
    <div>========== ./DEBUG =========</div>
    Files:<br />
    <ul>
        {#each files as file}
            <li>{file}</li>
        {/each}
    </ul>
    <br />
    Directories:<br />

    <div>========== ./ENDS =========</div>
</div>
