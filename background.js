// Register context menu
browser.runtime.onInstalled.addListener(() => {
  browser.contextMenus.create({
    id: "phash-image",
    title: "Compute Image Hash (Rust)",
    contexts: ["image"]
  });
});

// Handle clicks
browser.contextMenus.onClicked.addListener(async (info, tab) => {
  if (info.menuItemId !== "phash-image") return;

  // Inject code into the page to fetch the image and call WASM
  browser.tabs.executeScript(tab.id, {
    code: `
      (async () => {
        try {
          // Fetch image bytes
          const resp = await fetch("${info.srcUrl}");
          const bytes = new Uint8Array(await resp.arrayBuffer());

          // Load WASM module
          const wasmModule = await import(browser.runtime.getURL("wasm/pkg/wasm.js"));
          await wasmModule.default();
          const { process_image } = wasmModule;

          const hash = process_image(bytes);
          console.log("pHash:", hash);
          alert("pHash: " + hash);
        } catch(e) {
          console.error("Error computing hash:", e);
        }
      })();
    `
  });
});

