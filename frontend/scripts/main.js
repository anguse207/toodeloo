// const host = window.location.origin + /;
const host = "file:///home/gus/repos/toodeloo/frontend";
var userToken = "1234";

async function pulseContainer(element) {
  element.classList.add("pulse-text");
  setTimeout(() => {
    element.classList.remove("pulse-text");
  }, 400);
}

let lastScrollY = 0;
const header = document.getElementById("header");

window.addEventListener("scroll", () => {
  if (window.scrollY > lastScrollY) {
    header.classList.add("hidden");
  } else {
    header.classList.remove("hidden");
  }
  lastScrollY = window.scrollY;
});

// For dev purposes
document.addEventListener("DOMContentLoaded", function() {
  // clearCookies();
  setAuthToken(userToken);

  // Load the last list
  var lastListId = getCookie("last_list");
  if (lastListId) {
    loadList(lastListId);
  }
});