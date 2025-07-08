document.addEventListener("DOMContentLoaded", () => {
    // Get the base URL from the current location path
    const baseUrl = document.location.pathname.split('/').slice(0, -2).join('/');

    // Utility function to create and populate the version selector
    const createVersionSelector = (versions) => {
        const versionSelector = document.createElement("select");
        versionSelector.id = "version-selector";

        // Sort and iterate through the versions to populate the selector
        Object.entries(versions)
            .sort(([a], [b]) => (a === "latest" ? -1 : b === "latest" ? 1 : b.localeCompare(a, undefined, { numeric: true })))
            .forEach(([name, url]) => {
                const option = document.createElement("option");
                option.value = `${baseUrl}${url}`;
                option.textContent = name;
                // Pre-select the matching version
                option.selected = name === window.location.pathname.split('/')[2];
                versionSelector.appendChild(option);
            });

        // Redirect to the selected version when changed
        versionSelector.addEventListener("change", () => window.location.href = versionSelector.value);
        return versionSelector;
    };

    // Fetch the versions.json file and initialize the selector
    fetch(`${baseUrl}/versions.json`)
        .then(response => response.ok ? response.json() : Promise.reject(`Error: ${response.statusText}`))
        .then(versions => {
            // Locate the navigation element to append the version selector
            const nav = document.querySelector(".right-buttons");
            if (!nav) return console.error(".right-buttons element not found.");

            const versionBox = document.createElement("div");
            versionBox.id = "version-box";
            versionBox.appendChild(createVersionSelector(versions));
            nav.appendChild(versionBox);
        })
        .catch(error => console.error("Failed to fetch versions.json:", error));
});
