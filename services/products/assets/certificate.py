"""Render the public provenance projection; escape all user supplied text."""
import io
import json
import os
import sys
from datetime import date
from abc import ABC, abstractmethod
from xml.sax.saxutils import escape
from reportlab.lib import colors
from reportlab.lib.styles import ParagraphStyle
from reportlab.pdfbase import pdfmetrics
from reportlab.pdfbase.ttfonts import TTFont
from reportlab.platypus import SimpleDocTemplate, Paragraph, Spacer, Table, TableStyle

GREEN = colors.HexColor('#27543B')
INK = colors.HexColor('#20352A')
MUTED = colors.HexColor('#64766B')
PALE = colors.HexColor('#EEF4EC')
LINE = colors.HexColor('#DCE6DB')


class CertificateBuilder(ABC):
    """Builder contract: the director chooses order, the builder owns representation."""
    @abstractmethod
    def product(self, data): pass
    @abstractmethod
    def production(self, batch): pass
    @abstractmethod
    def materials(self, materials): pass
    @abstractmethod
    def disclaimer(self): pass
    @abstractmethod
    def result(self): pass


class ReportLabCertificateBuilder(CertificateBuilder):
    def __init__(self):
        font_dir = os.environ.get('CERTIFICATE_FONT_DIR', '/usr/share/fonts/truetype/dejavu')
        pdfmetrics.registerFont(TTFont('LocalBite', os.path.join(font_dir, 'DejaVuSans.ttf')))
        pdfmetrics.registerFont(TTFont('LocalBiteBold', os.path.join(font_dir, 'DejaVuSans-Bold.ttf')))
        pdfmetrics.registerFontFamily('LocalBite', normal='LocalBite', bold='LocalBiteBold')
        self.body = ParagraphStyle('body', fontName='LocalBite', fontSize=9, leading=14, textColor=INK, spaceAfter=3, wordWrap='CJK')
        self.label = ParagraphStyle('label', parent=self.body, fontSize=8, textColor=MUTED)
        self.heading = ParagraphStyle('heading', parent=self.body, fontName='LocalBiteBold', fontSize=14, leading=19, textColor=GREEN, spaceBefore=20, spaceAfter=10, keepWithNext=True)
        self.title = ParagraphStyle('title', parent=self.heading, fontSize=25, leading=32, spaceBefore=2, spaceAfter=12)
        self.story = []
        self.name = 'Product'

    def p(self, value, style=None):
        text = escape(str(value if value not in (None, '') else 'Not recorded')).replace('\n', '<br/>')
        return Paragraph(text, style or self.body)

    def fields(self, rows):
        table = Table([[self.p(k, self.label), self.p(v)] for k, v in rows], colWidths=[133, 350], hAlign='LEFT')
        table.setStyle(TableStyle([
            ('VALIGN', (0, 0), (-1, -1), 'TOP'),
            ('LEFTPADDING', (0, 0), (-1, -1), 10), ('RIGHTPADDING', (0, 0), (-1, -1), 10),
            ('TOPPADDING', (0, 0), (-1, -1), 8), ('BOTTOMPADDING', (0, 0), (-1, -1), 8),
            ('ROWBACKGROUNDS', (0, 0), (-1, -1), [PALE, colors.white]),
            ('LINEBELOW', (0, 0), (-1, -1), .4, LINE),
        ]))
        return table

    def product(self, data):
        product = data['product']
        self.name = product['name']
        self.story += [self.p('PRODUCT TRACEABILITY RECORD', self.label), self.p(self.name, self.title)]
        if product.get('description'):
            self.story += [self.p(product['description']), Spacer(1, 12)]
        self.story += [self.fields([
            ('Producer', data.get('farm_name') or 'Information unavailable'),
            ('Product type', product['product_type']), ('Product expiry', product.get('expiry_date')),
            ('Traceability ID', product['qr_token']),
        ])]

    def production(self, batch):
        self.story += [self.p('01  Production journey', self.heading)]
        if not batch:
            self.story += [self.p('Production information is temporarily unavailable.')]
            return
        self.story += [self.fields([
            ('Batch', batch['name']),
            ('Status', batch['status'].replace('_', ' ').title()),
            ('Production start', batch.get('start_date')), ('Production end', batch.get('end_date')),
        ])]
        for step in batch.get('steps', []):
            self.story += [self.p(str(step['step_order']) + '. ' + step['name'])]
            if step.get('description'):
                self.story += [self.p(step['description'])]

    def materials(self, materials):
        self.story += [self.p('02  Raw materials used', self.heading)]
        if not materials:
            self.story += [self.p('No raw materials recorded for this batch.')]
        for index, material in enumerate(materials, 1):
            style = ParagraphStyle('material', parent=self.body, fontName='LocalBiteBold', fontSize=11, leading=16, spaceBefore=14, spaceAfter=6, keepWithNext=True)
            self.story += [self.p(f"{index:02d}  {material['name']}", style), self.fields([
                ('Origin', material.get('origin')), ('Harvest / production date', material.get('harvest_date')),
            ])]

    def disclaimer(self):
        self.story += [Spacer(1, 22), self.p('This record reflects information entered by the producer. Dates marked “Not recorded” have not been provided.', self.label)]

    @staticmethod
    def page(canvas, document):
        canvas.saveState()
        canvas.setFillColor(GREEN)
        canvas.rect(0, 776, 595.28, 66, stroke=0, fill=1)
        canvas.setFont('LocalBiteBold', 16)
        canvas.setFillColor(colors.white)
        canvas.drawString(56, 805, 'LOCAL BITE')
        canvas.setFont('LocalBite', 8)
        canvas.drawRightString(539, 807, 'FROM FARM TO TABLE')
        canvas.setStrokeColor(LINE)
        canvas.line(56, 48, 539, 48)
        canvas.setFillColor(MUTED)
        canvas.setFont('LocalBite', 7)
        canvas.drawString(56, 32, f'Generated {date.today().isoformat()} | Producer traceability record')
        canvas.drawRightString(539, 32, f'Page {document.page}')
        canvas.restoreState()

    def result(self):
        output = io.BytesIO()
        doc = SimpleDocTemplate(output, pagesize=(595.28, 841.89), rightMargin=56, leftMargin=56, topMargin=91, bottomMargin=66,
                                title=f'Local Bite - {self.name}', author='Local Bite')
        doc.build(self.story, onFirstPage=self.page, onLaterPages=self.page)
        return output.getvalue()


class CertificateDirector:
    def build(self, data, builder):
        builder.product(data)
        builder.production(data.get('batch'))
        builder.materials((data.get('batch') or {}).get('raw_materials', []))
        builder.disclaimer()
        return builder.result()


def render(data):
    return CertificateDirector().build(data, ReportLabCertificateBuilder())


if __name__ == '__main__':
    sys.stdout.buffer.write(render(json.load(sys.stdin)))
