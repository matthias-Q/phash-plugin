browser.runtime.onInstalled.addListener(() => {
  browser.contextMenus.create({
    id: "phash-image",
    title: "Compute Image Hash (Rust)",
    contexts: ["image"]
  });
});

browser.contextMenus.onClicked.addListener((info, tab) => {
  if (info.menuItemId !== "phash-image") return;

  const code = `
    (function(imageUrl) {
      const doFetch = async () => {
        try {
          const resp = await fetch(imageUrl);
          const bytes = new Uint8Array(await resp.arrayBuffer());
          const wasmModule = await import(browser.runtime.getURL("wasm/pkg/wasm.js"));
          await wasmModule.default();
          const { process_image } = wasmModule;
          const hash = process_image(bytes);
          if (hash.startsWith("error:")) {
              console.error(hash);
          } else {
              console.log("pHash:", hash);
              alert("pHash: " + hash);
          }
        } catch(e) {
          console.error("Error computing hash:", e);
        }
      };
      doFetch();
    })("${info.srcUrl}");
  `;

  browser.tabs.executeScript(tab.id, { code });
});

