const urlBase = "http://localhost:3030";

class BackendURLBuilder {
    static getAccountById(id) {
        return urlBase + "/account/id/" + id + "/";
    }

    static createAccount() {
        return urlBase + "/signup";
    }

    static login() {
        return urlBase + "/login";
    }

    static logout() {
        return urlBase + "/logout";
    }

    static withPath(path) {
        return urlBase + path;
    }

    static whois() {
        return urlBase + "/whois"
    }

    static changePassword() {
        return urlBase + "/change-password"
    }
}

export default BackendURLBuilder;
