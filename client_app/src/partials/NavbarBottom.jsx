import React, { Component, Fragment } from 'react';
import PropTypes from 'prop-types';
import {
  Navbar,
  Nav,
  NavItem
} from 'reactstrap';
import { Link } from 'react-router-dom';
import TiHome from 'react-icons/lib/ti/home';
import MdSettings from 'react-icons/lib/md/settings';
import MdAssignment from 'react-icons/lib/md/assignment';
import IoRadioWaves from 'react-icons/lib/io/radio-waves';
import CssModules from 'react-css-modules';

import styles from '../css/NavbarBottom.css';
class NavbarBottom extends Component {
  constructor(props) {
    super(props);

    this.state = {
      homePage: "",
      homeLink: "",
      postPage: "",
      postLink: "",
      offerPage: "",
      offerLink: "",
      settingPage: "",
      settingLink: ""
    };
  }

  componentDidMount() { 
    if (this.props.currentRoute == "/home" || this.props.currentRoute == "/search") {
      this.setState(this.state)
      this.setState({
        homePage: "active-menu",
        homeLink: this.props.currentRoute
      })
    }
    else if (this.props.currentRoute == "/post") {
      this.setState(this.state)
      this.setState({
        postPage: "active-menu",
        postLink: this.props.currentRoute
      })
    }
  }

  render() {    
    return (
      <Navbar fixed="bottom" light expand="md" styleName="NavbarBottom">
        <Nav>
          <NavItem styleName="nav-item">
            <Link to={this.state.homeLink} styleName={this.state.homePage}><TiHome /><span>Home</span></Link>
          </NavItem>
          <NavItem styleName="nav-item">
            <Link to="#" styleName={this.state.postPage}><IoRadioWaves /><span>My Ad</span></Link>
          </NavItem>
          <NavItem styleName="nav-item">
            <Link to="#" styleName={this.state.offerPage}><MdAssignment /><span>Offer</span></Link>
          </NavItem>
          <NavItem styleName="nav-item">
            <Link to="#" styleName={this.state.settingPage}><MdSettings /><span>Account</span></Link>
          </NavItem>
        </Nav>
      </Navbar>
    )
  }
}

export default CssModules(NavbarBottom, styles, { allowMultiple: true });
