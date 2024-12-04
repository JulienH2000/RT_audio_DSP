function Band(f, a, q) {
    this.f = f;
    this.f_d;
    this.a = a;
    this.a_d;
    this.q = q;
    this.q_d;
}

Band.prototype.Updatef = function(f) {
    this.f = f;
    this.f_d = f + 'Hz';
}

Band.prototype.Updatea = function(a) {
    this.a = a;
    this.a_d = a + 'dB';
}

Band.prototype.Updateq = function(q) {
    this.q = q;
    this.q_d = q;
}