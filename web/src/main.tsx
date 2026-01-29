import React from 'react'
import ReactDOM from 'react-dom/client'
import App from './App.tsx'
import {DevSupport} from "@react-buddy/ide-toolbox";
import {ComponentPreviews, useInitial} from "./dev";
import {BrowserRouter} from 'react-router-dom';

ReactDOM.createRoot(document.getElementById('root')!).render(
    <React.StrictMode>
        <BrowserRouter>
            <DevSupport ComponentPreviews={ComponentPreviews} useInitialHook={useInitial}>
                <App/>
            </DevSupport>
        </BrowserRouter>
    </React.StrictMode>,
)