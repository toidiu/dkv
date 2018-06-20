import * as React from 'react';
import { render } from 'react-dom';
import { BrowserRouter, HashRouter } from 'react-router-dom'
import App from './pages/App.tsx'


render((
  <HashRouter>
    <App />
  </HashRouter>
), document.getElementById('app'));

