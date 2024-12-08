function setLastListCookie(listID) {
    document.cookie = "last_list=" + listID;
}

function setAuthToken(token) {
    document.cookie = "auth_token=" + token;
}

function printCookies() {
    console.log(document.cookie);
}

function getCookie(name) {
    const cookies = document.cookie.split("; ");
    for (const cookie of cookies) {
        const [key, value] = cookie.split("=");
        if (key === name) {
            return value;
        }
    }
    return null; // Return null if the cookie doesn't exist
}

function clearCookies() {
    document.cookie.split(";").forEach(cookie => {
        const name = cookie.split("=")[0].trim();
        document.cookie = `${name}=; expires=Thu, 01 Jan 1970 00:00:00 GMT; path=/`;
    });
}