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

    static withPath(path) {
        return urlBase + path;
    }
}

export default BackendURLBuilder;
