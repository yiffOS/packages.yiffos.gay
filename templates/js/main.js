document.getElementById('search').addEventListener('keyup', function(e) {
    if (e.key === "Enter") {
        window.find(this.value);
    }
});