import * as React from 'react';
import './Nav.less';

export interface NavProps {
  title: string;
}

function Nav(props: NavProps) {

  return (
    <nav className="nav">
      <div>the title is {props.title}</div>
    </nav>
  );
}


export default Nav;
