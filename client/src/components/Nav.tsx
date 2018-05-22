import * as React from 'react';
import './Nav.less';

export interface NavProps {
  title: string;
  links: Array<number>;
}

function Nav(props: NavProps) {

  return (
    <nav className="nav">
      <a href="example.com"><strong>{props.title}</strong></a>

      {props.links
        .map(link =>
          <Link
            key={link}
            name={link.split(':')[0]}
            class={link.split(':')[1]}
            url="example.com"
          />
        )
      }

    </nav>
  );
}

export interface LinkProps {
  name: string;
  class: string;
  url: string;
}

function Link(props: LinkProps) {
  return (
    <a className={props.class} href={props.url}>{props.name}</a>
  )
}


export default Nav;
