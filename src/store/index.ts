// store.js
import { reactive } from 'vue'

export const store = reactive({
    currentId: "",
    setCurrentId(id: string) {
        this.currentId = id;
    },
})