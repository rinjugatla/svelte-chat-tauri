import { doc, collection, addDoc, query, onSnapshot, orderBy, getDoc, getDocs, where } from 'firebase/firestore';
import { db, getCurrentUserInfo } from './firebase';
import dayjs from 'dayjs';
import { FirebaseError } from 'firebase/app';
import { SnapshotChats } from './store';

/**
 * 部屋IDを取得
 * @param {string} name 部屋名
 */
const getRoomIdByName = async (name = '') => {
	try {
		// すでに部屋が作成済みかチェック
		const q = query(
			collection(db, 'rooms'),
			where('name', '==', name));
		const querySnapshot = await getDocs(q);
		const exists = querySnapshot.size > 0;
		if (!exists) { return null; }

		/**
		 * @type {string[]}
		 */
		let ids = [];
		querySnapshot.forEach((doc) => {
			ids.push(doc.id);
		});
		return ids[0];
	} catch(e){
		return null;
	}
}

/**
 * 部屋が存在するか
 * @param {string} id 部屋ID
 * @returns 
 */
export const existRoomById = async(id = '') => {
	try{
		// すでに部屋が作成済みかチェック
		const docRef = doc(db, 'rooms', id);
		const querySnapshot = await getDoc(docRef);
		const exists = querySnapshot.exists();
		return exists;
	}
	catch (e) {
		return false;
	}
}

/**
 * 部屋を登録
 * @param {string} name 作成する部屋名
 * @returns 部屋ID
 */
export const postRoom = async (name = '') => {
	try {
		const userInfo = getCurrentUserInfo();
		if (userInfo == null){ return null; }

		const roomId = await getRoomIdByName(name);
		const exists = roomId != null;
		if (exists){ return roomId; }

		const doc = await addDoc(collection(db, 'rooms'), {
			name: name,
			created_by: userInfo?.uid,
			created_at: dayjs().format('YYYY/MM/DD HH:mm:ss')
		});

		const registedRoomId = doc.id;
		return registedRoomId;
	} catch(e) {
		if (e instanceof FirebaseError) {
			console.log(e);
		}
	}
}

/**
 * チャットを登録
 * @param roomId string 部屋ID
 * @param message string メッセージ 
 * @returns チャットID
 */
export const postMessage = async (roomId = '', message = '') => {
	try {
		const userInfo = getCurrentUserInfo();
		if (userInfo == null){ return; }

		const docRef = await addDoc(collection(db, 'rooms', roomId, 'chats'), {
			message: message,
			uid: userInfo?.uid,
			created_at: dayjs().format('YYYY/MM/DD HH:mm:ss')
		});

		return docRef.id;
	} catch (e) {
		if (e instanceof FirebaseError) {
			console.log(e);
		}
	}
};

/**
 * チャット履歴をリアルタイムに取得
 * @param roomId string 部屋ID
 * @returns 履歴取得停止
 */
export const onSnapshotChats = (roomId = '') => {
    const q = query(
        collection(db, 'rooms', roomId, 'chats'),
        orderBy('created_at', 'desc'));
	const unsubscribe = onSnapshot(q, (querySnapshot) => {
        /**
         * @type {Array.<{id: string, message: string, time: string}>}
         */
        let chats = [];
		querySnapshot.forEach((doc) => {
            const data = doc.data();
			console.log(data);
			chats.push({id: doc.id, message: data.message, time: data.created_at});
		});
        SnapshotChats.set(chats);
	});
    return unsubscribe;
};
