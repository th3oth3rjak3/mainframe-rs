import Toast, { POSITION, type PluginOptions } from 'vue-toastification';

import 'vue-toastification/dist/index.css';

export const options: PluginOptions = {
    position: POSITION.BOTTOM_RIGHT,
    timeout: 5000,
    closeOnClick: true,
    pauseOnFocusLoss: true,
    pauseOnHover: true,
    draggable: true,
    draggablePercent: 0.6,
    showCloseButtonOnHover: false,
    hideProgressBar: true,
    closeButton: "button",
    icon: true,
    rtl: false
};

export const plugin = Toast;