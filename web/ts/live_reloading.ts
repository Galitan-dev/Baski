const poll = () => {
    fetch("/poll/live_reloading")
        .then(res => res.text())
        .then(action => {
            if (action === "reload") window.location.reload();
            else setTimeout(poll, 100);
        });
}

poll();