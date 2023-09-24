import { writable } from "svelte/store";
import Cookies from 'js-cookie'

const cookieUid = Cookies.get("uid");
export const UserId = writable(cookieUid ?? "");
// チャット履歴のリアルタイム取得結果
export const SnapshotChats = writable([{id: "", message: "", time: ""}]);