var name = "KissAnime";
var animeName = document.getElementById("navsubbar").children[0].children[0].innerText;
var animeName = animeName.trim().substr(6, animeName.length - 18);
var episode = document.getElementById("selectEpisode");
var episode = episode.children[episode.selectedIndex].innerText.trim();
var episode = episode.substr(8, episode.length - 8);
var episode = parseInt(episode, 10);

document.getElementById("my_video_1_html5_api").onplay = function() {
    browser.runtime.sendMessage({
        name: name,
        state: "Episode " + episode,
        details: "Watching " + animeName,
        start_time: Math.floor(Date.now() / 1000)
    });
};

document.getElementById("my_video_1_html5_api").onpause = function() {
    browser.runtime.sendMessage({
        clear: true
    });
}