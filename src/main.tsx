import 'vite/modulepreload-polyfill';
import React from 'react';
import ReactDOM from 'react-dom/client';
import AppRoutes from 'routes';
import { BrowserRouter } from 'react-router-dom';
import { Layout } from 'components';
import { checkEnv } from 'utils';

import './style/global.scss';
import './style/tailwind.css';

console.log(checkEnv()); // are we in prod or dev?

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <BrowserRouter>
      <Layout>
        <AppRoutes />
      </Layout>
    </BrowserRouter>
  </React.StrictMode>,
);
