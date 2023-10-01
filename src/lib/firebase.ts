import { initializeApp } from "firebase/app";
import { getFirestore } from "firebase/firestore";
import { getStorage } from "firebase/storage";
import { PUBLIC_FIREBASE_API_KEY, PUBLIC_FIREBASE_AUTH_DOMAIN, PUBLIC_FIREBASE_PROJECT_ID, PUBLIC_FIREBASE_STORAGE_BUCKET, PUBLIC_FIREBASE_MESSAGING_SENDER_ID, PUBLIC_FIREBASE_APP_ID } from "$env/static/public";
import { getAuth, signOut, signInWithPopup, GoogleAuthProvider } from "firebase/auth";
import { UserId } from "$lib/store";
import Cookies from 'js-cookie'

const firebaseConfig = {
    apiKey: PUBLIC_FIREBASE_API_KEY,
    authDomain: PUBLIC_FIREBASE_AUTH_DOMAIN,
    projectId: PUBLIC_FIREBASE_PROJECT_ID,
    storageBucket: PUBLIC_FIREBASE_STORAGE_BUCKET,
    messagingSenderId: PUBLIC_FIREBASE_MESSAGING_SENDER_ID,
    appId: PUBLIC_FIREBASE_APP_ID
};

const app = initializeApp(firebaseConfig);
export const db = getFirestore(app);

const provider = new GoogleAuthProvider();
const auth = getAuth();

export const storage = getStorage(app);

export const signInWithGoogle = () => {
    // signInWithRedirect(auth, provider);
    signInWithPopup(auth, provider)
        .then((result) => {
            const credential = GoogleAuthProvider.credentialFromResult(result);
            if (credential == null ){return;}

            const user = result.user;
            UserId.set(user.uid);
            Cookies.set("uid", user.uid, { expires: 3 });
            document.location.reload();
        }).catch((error) => {
            const code = error.code;
            const message = error.message;
            GoogleAuthProvider.credentialFromError(error);
            alert(`${code} ${message}`);
        });
}

export const signOutWithGoogle = () => {
    signOut(auth).then(() => {
        Cookies.remove("uid");
        document.location.reload();
    }).catch((error) => {
        alert(`ログアウトに失敗しました。\n${error}`);
    });
}

export const getCurrentUserInfo = () => {
    if (auth == null){ return null; }
    const info = {
        displayName: auth.currentUser?.displayName,
        email: auth.currentUser?.email,
        uid: auth.currentUser?.uid
    }

    return info;
}