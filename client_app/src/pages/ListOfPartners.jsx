import React, { Component, Fragment } from 'react';
import {
  Container, Row, Col,
  Card, CardImg, CardText, CardBody,
  CardTitle, CardSubtitle, CardLink,
  Button
} from 'reactstrap';
import CssModules from 'react-css-modules';
import { Link } from 'react-router-dom';
import NavbarMain from '../partials/NavbarMain';
import NavbarBottom from '../partials/NavbarBottom';
import OfferForm from '../partials/OfferForm';
import TiFlashOutline from 'react-icons/lib/ti/flash-outline'

import dummyImg from '../img/dummy-img.png'
import styles from '../css/ListOfPartners.css';

class ListOfPartners extends Component {
  constructor(props) {
    super(props)
    this.state = {
      showOfferForm: false
    }
  }

  toggleOfferForm = () => {
    this.setState({ showOfferForm: !this.state.showOfferForm })
  }

  renderOfferForm() {
    return <OfferForm toggleOfferForm={this.toggleOfferForm} />
  }

  renderPartners() { 
    return (
      <Container className="basic-container">
        {[...Array(10)].map((_, index) =>
          <Row key={index} styleName="box">
            <Col md={{ size: 4, offset: 4 }}>
              <Card>
                <CardImg top width="100%" src={dummyImg} alt="Card image cap" />
                <CardBody>
                  <CardTitle>Tandry Syawaludin Soedijanto</CardTitle>
                  <CardSubtitle styleName="trash-category"><span>Sampah Plastik PVC</span></CardSubtitle>
                  <CardText styleName="caption">Jalan Duku 1 Blok C2/25 Pondok Sejahtera, Kelurahan Kutabaru, Kecamatan Pasar Kemis, Kabupaten Tangerang, Banten, 11561</CardText>
                  <CardLink href="#" onClick={() => this.toggleOfferForm()} styleName="action-menu"><TiFlashOutline /><span>Offer</span></CardLink>
                </CardBody>
              </Card>
            </Col>
          </Row>
        )}
        <Row>
          <Col md={{ size: 4, offset: 4 }}>
            <Button outline block size="sm">Load More</Button>
          </Col>
        </Row>
      </Container>
    )
  }

  render() {
    return (
      <div styleName="ListOfPartners">
        <NavbarMain prevRoute={this.props.history.goBack} />
        {this.state.showOfferForm ? this.renderOfferForm() : this.renderPartners()}
        <NavbarBottom currentRoute={this.props.location.pathname} />  
      </div>
    )
  }
}
export default CssModules(ListOfPartners, styles, { allowMultiple: true });