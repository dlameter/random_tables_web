const urlBase = "http://localhost:3030";

class BackendURLBuilder {
    static getAccountById(id) {
        return urlBase + "/account/id/" + id + "/";
    }
}

export default BackendURLBuilder;
