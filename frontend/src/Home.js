import { useAuth } from "./auth";

function Home() {
    const auth = useAuth();

    return (
        <div>
            <h1>
                {
                    auth.user ?
                        "Welcome " + auth.user.username + " to Random Tables Web!"
                        :
                        "Welcome to Random Tables Web!"
                }
            </h1>
        </div>
    );
}

export default Home;
