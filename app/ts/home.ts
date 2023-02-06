const searchInput = document.getElementById("search-input") as HTMLInputElement;
const searchSubmitButtons = document.getElementsByClassName("search-submit-button") as HTMLCollectionOf<HTMLButtonElement>;

const isDisabled = () => searchInput.value.length === 0;

searchInput.addEventListener("keydown", (event) => {
    if (isDisabled() && event.key === "Enter") event.preventDefault();
})

searchInput.addEventListener("keyup", () => {
    console.log(isDisabled())

    for (let i = 0; i < searchSubmitButtons.length; i++) {
        const button = searchSubmitButtons.item(i);
        if (button) button.disabled = isDisabled();
    }
});
