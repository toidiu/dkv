import * as React from 'react';
import Nav from './Nav.tsx'

export interface AppProps {
  title: string;
  name: string;
}

function App(props: AppProps) {

  return (
    <div>
      <Nav title={props.title} links={["github", "about:active"]} />



      <h1>my name is {props.name}</h1>
    </div>
  );
}

export default App;
