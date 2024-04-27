import {createApp} from "vue";
import "./styles.css";
import MainUI from "./MainUI.vue";
import {FontAwesomeIcon} from "@fortawesome/vue-fontawesome";
import {library} from "@fortawesome/fontawesome-svg-core";
import {fas} from '@fortawesome/free-solid-svg-icons';

library.add(fas);
const app = createApp(MainUI);
app.component('font-awesome-icon', FontAwesomeIcon);
app.mount("#main_ui");

