document.getElementById('search').addEventListener('keyup', function(e) {
    if (e.keyCode === 13) {
        window.find(this.value);
    }
});