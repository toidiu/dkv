import * as React from 'react';
import './Nav.less';

function Nav(props: NavProps) {

  return (
    <nav className="nav">

      <ul>
        <li><a href="#"><strong>{props.title}</strong></a></li>

        <li><a href="#todo">todo</a></li>
      </ul>

    </nav>
  );
}

export default Nav;
