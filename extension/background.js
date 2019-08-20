var socket = new WebSocket("ws://127.0.0.1:43069");
var map = new Map();

socket.onopen = function(e) {
    console.log("Connected to WebSocket server!")

    browser.runtime.onMessage.addListener(onMessage);
}

function onMessage(message) {
    if (message.clear) {
        socket.send(JSON.stringify({
            clear: true
        }));
        return;
    }

    socket.send(JSON.stringify({
        name: message.name,
        state: message.state,
        details: message.details
    }));

    function logTabs(tabs) {
        console.log(tabs);

        if (!map.has(tabs[0].id)) {
            console.log("Adding tab to map");
            map.set(tabs[0].id, "");
        }
    }

    function onError(error) {
        console.log(`Error: ${error}`);
    }

    var querying = browser.tabs.query({currentWindow: true, active: true});
    querying.then(logTabs, onError);
}

browser.tabs.onUpdated.addListener(onTabUpdated);
browser.tabs.onRemoved.addListener(onTabRemoved);

function onTabUpdated(tabId, changeInfo, tabInfo) {
    if (changeInfo.url) {
        if (map.has(tabId)) {
            socket.send(JSON.stringify({
                clear: true
            }));1

            map.delete(tabId);
        }
    }

}

function onTabRemoved(tabId) {
    if (map.has(tabId)) {
        socket.send(JSON.stringify({
            clear: true
        }));

        map.delete(tabId);
    }
}

browser.windows.onRemoved.addListener(onRemoved);

function onRemoved(windowID) {
    var gettingAll = browser.windows.getAll();
    gettingAll.then(then, error);
}

function then(array) {
    if (array == undefined || array.length == 0) {
        socket.send(JSON.stringify({
            clear: true
        }));
    }
}

function error(error) {
    console.log("Error: " + error)
}