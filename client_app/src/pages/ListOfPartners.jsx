import React, { Component, Fragment } from 'react';
import {
  Container, Row, Col,
  Card, CardImg, CardText, CardBody,
  CardTitle, CardSubtitle, CardLink,
  Button
} from 'reactstrap';
import CssModules from 'react-css-modules';
import { Link } from 'react-router-dom';
import { map } from 'lodash';
import { FiZap } from "react-icons/fi";
import axios from 'axios';

import NavbarMain from '../partials/NavbarMain';
import NavbarBottom from '../partials/NavbarBottom';
import OfferForm from '../partials/OfferForm';
import dummyImg from '../img/dummy-img.png'
import styles from '../css/ListOfPartners.css';

class ListOfPartners extends Component {
  constructor(props) {
    super(props)
    this.state = {
      showOfferForm: false,
      currentPage: 0
    }

    this.handleLoadMore = this.handleLoadMore.bind(this);
    this.handleCancelOffer = this.handleCancelOffer.bind(this);
  }

  componentWillMount() {
    this.getPartners();
  }

  getPartners() {
    this.setState({ currentPage: this.state.currentPage + 1 });
    axios({
      method: 'GET',
      url: 'http://localhost:8000/partners/' + this.state.currentPage
    })
    .then(response => {
      
    })
    .catch(error => {
      console.log(error);
    })
    .then(() => {

    });
  }

  renderPartners() {
    return (
      <Row className="box">
        <Col md={{ size: 4, offset: 4 }}>
          <Card>
            <CardImg top width="100%" src={dummyImg} alt="Card image cap" />
            <CardBody>
              <CardTitle>Tandry Syawaludin Soedijanto</CardTitle>
              <CardSubtitle className="trash-category"><span>Sampah Plastik PVC</span></CardSubtitle>
              <CardText className="caption">Jalan Duku 1 Blok C2/25 Pondok Sejahtera, Kelurahan Kutabaru, Kecamatan Pasar Kemis, Kabupaten Tangerang, Banten, 11561</CardText>
              <CardLink href="#" onClick={this.toggleOfferForm.bind(this)} className="action-menu"><FiZap /><span>Offer</span></CardLink>
            </CardBody>
          </Card>
        </Col>
      </Row>
    )
  }

  handleLoadMore() {
    this.getPartners();
  }

  handleCancelOffer() {
    this.setState({ showOfferForm: !this.state.showOfferForm })
  }

  renderOfferForm() {
    return <OfferForm handleCancelOffer={this.handleCancelOffer} />
  }

  renderPartners() { 
    return (
      <div styleName="ListOfPartners">      
        <Container className="basic-container">
          {this.renderPartners}
          <Row>
            <Col md={{ size: 4, offset: 4 }}>
              <Button outline block size="sm" onClick={this.handleLoadMore}>Load More</Button>
            </Col>
          </Row>
        </Container>
      </div>
    )
  }

  render() {
    return (
      <Fragment>
        <NavbarMain prevRoute={this.props.history.goBack} currentRoute={this.props.location.pathname} />
        {this.state.showOfferForm ? this.renderOfferForm() : this.renderPartners()}
        <NavbarBottom currentRoute={this.props.location.pathname} />  
      </Fragment>
    )
  }
}
export default CssModules(ListOfPartners, styles, { allowMultiple: true });