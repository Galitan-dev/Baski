const ws = new WebSocket(`ws://${window.location.host}/live_reloading/subscribe`);

ws.onmessage = () => {
    window.location.reload();
};

ws.onopen = () => {
    console.log("%c⚡️ Live Reloading %cis %cenabled%c!", "color: #D90; font-weight: bold", "color: #D90", "color: #0D5", "color: #D90");
}
