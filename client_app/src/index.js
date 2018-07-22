import React from 'react';
import ReactDOM from 'react-dom';
import App from './App';  
import registerServiceWorker from './registerServiceWorker';
import WebFont from 'webfontloader';

import 'bootstrap/dist/css/bootstrap.min.css';
import './css/index.css';
WebFont.load({
  google: {
    families: ['Open Sans', 'sans-serif']
  }
});

ReactDOM.render( < App / > , document.getElementById('root'));
registerServiceWorker();