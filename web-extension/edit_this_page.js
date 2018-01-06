/*
On startup, connect to the "edit_this_page" app.
*/
var port = browser.runtime.connectNative("edit_this_page");

var myWindowId;

/*
Listen for messages from the app.
*/
port.onMessage.addListener((response) => {
  console.log("Received: " + response);
});

browser.windows.getCurrent({populate: true}).then((windowInfo) => {
  myWindowId = windowInfo.id;
  updateContent();
});

/*
On a click on the browser action, send the app the 'edit' instruction.
*/
browser.browserAction.onClicked.addListener(() => {
  browser.tabs.query({windowId: myWindowId, active: true}).then((tabs) => {
      console.log("Editing Page: " + tabs[0].url);
      port.postMessage("edit: " + tabs[0].url);
  });

});
