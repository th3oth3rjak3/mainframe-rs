import type { JSX } from "solid-js";

export default function Layout(props?: { children?: JSX.Element }) {
    return (
        <div>
            <p>layout works</p>
            {props?.children}
        </div>
    );
}
