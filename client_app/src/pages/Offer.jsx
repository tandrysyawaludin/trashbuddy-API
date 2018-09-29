import React, { Component, Fragment } from 'react'
import {
  ListGroup,
  ListGroupItem,
  Row,
  Col,
  Container,
  Button
} from 'reactstrap'
import { Link } from 'react-router-dom'
import CSSModules from 'react-css-modules'

import NavbarBottom from '../partials/NavbarBottom'
import NavbarMain from '../partials/NavbarMain'
import styles from '../css/Offer.css'

import { Auth } from '../helper/CheckAuth'

class Offer extends Component {
  constructor(props) {
    super(props)
  }

  render() {
    return (
      <Fragment>
        <NavbarMain prevRoute={this.props.history.goBack} currentRoute={this.props.location.pathname} />
        <div styleName="Offer">
          <Container className="basic-container">
            <Row>
              <Col md={{ size: 6, offset: 3 }}>
                <ListGroup>
                  <ListGroupItem tag="a" href="/">Tandry</ListGroupItem>
                  <ListGroupItem tag="a" href="/">Tandry 2</ListGroupItem>
                </ListGroup>
              </Col>
            </Row>
          </Container>
        </div>
        <NavbarBottom currentRoute={this.props.location.pathname} />
      </Fragment>
    )
  }
}

export default CSSModules(Offer, styles, { allowMultiple: true })
